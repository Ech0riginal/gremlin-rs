#!/usr/bin/env python3

"""
The file this thing outputs isn't perfect - integers will be missing their signage, etc.
but it does the trick for the most part. Thank you TinkerPop team for making it easy to ingest:
this'll get you pretty close to comprehensive coverage of the spec.
"""

def module(name: str) -> str:
    return f"""
mod {name} {{
    use super::TestCase;
    use serde_json::json;
    use gremlin_client::prelude::*;

"""

def static_name(name: str) -> str:
    return name.upper().replace(' ', '_')


if __name__ == '__main__':
    from bs4 import BeautifulSoup
    from json import loads
    from requests import get
    from re import sub

    modules = []
    engines = {
        '2d0': 'V2',
        '3d0': 'V3',
    }

    html = get("https://tinkerpop.apache.org/docs/3.4.1/dev/io")
    soup = BeautifulSoup(html.content, "html.parser")
    versions = soup.find_all('div', attrs={'class': 'sect1'})[:-1]

    for version in versions:
        version_name = list(filter(lambda el: el.text != '\n', version.children))[0].attrs.get('id').replace('graphson-', '')
        if version_name == '1d0':
            continue

        mod = module(f'v_{version_name}')
        sections = version.find_all('div', attrs={'class': 'sect2'})
        seen = []
        tests = []
        test_cases = []
        for section in sections:
            components = section.find_all('div', attrs={'class': 'sect3'})
            for component in components:
                component_name = list(filter(lambda el: el.text != '\n', component.children))[0].text
                if component_name is None or component_name in seen:
                    continue
                else:
                    seen.append(component_name)
                component_name = static_name(component_name)
                # The JSON posted on the site isn't always valid. Sad.
                graphson = component.find('code', attrs={'data-lang': 'json'}).text
                graphson = graphson.replace('\n', '').strip()
                graphson = sub(r"\s+", " ", graphson)
                graphson = sub(r'(")\s+(")', r'\1, \2', graphson)
                # Ensure it's valid JSON
                if loads(graphson) is None:
                    continue
                test_cases.append(f"""
    lazy_static::lazy_static! {{
        pub static ref {component_name}: TestCase = TestCase {{
            serial: json!({graphson}),
            object: GValue::Null,
        }};
    }}""")
        for component_name in seen:
            component_name = static_name(component_name)
            engine_name = engines[version_name]
            tests.append(f'    super::test!({component_name.lower()}, {engine_name}, {component_name});')
        mod += '\n'.join(tests)
        mod += '\n'
        mod += '\n'.join(test_cases)
        mod += "\n}\n"
        modules.append(mod)

    with open('generated_tests.rs', 'w') as f:
        f.write("""
use serde_json::Value;
use gremlin_client::prelude::*;

pub struct TestCase {
    pub serial: Value,
    pub object: GValue,
}

impl TestCase {
    pub fn test<DS: GraphSON>(&self) {
        self.deserialize::<DS>();
        self.serialialize::<DS>();
    }

    pub fn deserialize<DS: GraphSON>(&self) {
        let result = DS::deserialize(&self.serial);
        assert!(result.is_ok(), "Deserialization failed");
        assert_eq!(self.object, result.unwrap(), "Deserialization doesn't match expectation");
    }

    /// I had a stroke typing this but its great so it stays
    pub fn serialialize<DS: GraphSON>(&self) {
        let result = DS::serialize(&self.object);
        assert!(result.is_ok(), "Serialization failed");
        assert_eq!(self.serial, result.unwrap(), "Serialization doesn't match expectation");
    }
}

macro_rules! test {
    ($fun:ident, $engine:ident, $case:ident) => {
        #[test]
        fn $fun() {
            $case.test::<gremlin_client::prelude::$engine>();
        }
    };
}

pub(self) use test;

    
    """)
        for mod in modules:
            f.write(mod)
