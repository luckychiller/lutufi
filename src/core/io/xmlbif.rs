use std::path::Path;
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, BytesText, Event};
use quick_xml::Reader;
use quick_xml::Writer;
use crate::core::{
    domain::Domain,
    error::{LutufiError, LutufiResult},
    factor::ConditionalProbabilityTable,
    models::bayesian_network::BayesianNetwork,
    variable::Variable,
};

/// Reader and writer for the XML Bayesian Interchange Format (XMLBIF).
pub struct XmlBifFormat;

impl XmlBifFormat {
    /// Exports a Bayesian network to an XMLBIF-formatted string.
    pub fn export(network: &BayesianNetwork) -> LutufiResult<String> {
        let mut writer = Writer::new_with_indent(Vec::new(), b' ', 2);

        let _ = writer
            .write_event(Event::Decl(BytesDecl::new("1.0", Some("UTF-8"), None)));
        let _ = writer
            .write_event(Event::Start(BytesStart::new("BIF").with_attributes([("VERSION", "0.3")])));
        let _ = writer
            .write_event(Event::Start(BytesStart::new("NETWORK")));

        let _ = write_text_element_quiet(&mut writer, "NAME", "lutufi_model");

        let variables = network.variables();
        let node_names: Vec<&str> = network.nodes();

        for var_name in &node_names {
            let var = network.variable(var_name).map_err(|_| {
                LutufiError::SerializationError {
                    reason: format!("Variable '{}' not found", var_name),
                }
            })?;
            let domain = var.domain();

            let _ = writer.write_event(Event::Start(
                BytesStart::new("VARIABLE").with_attributes([(
                    "TYPE",
                    match domain {
                        Domain::Continuous { .. } => "continuous",
                        _ => "discrete",
                    },
                )]),
            ));
            let _ = write_text_element_quiet(&mut writer, "NAME", var_name);

            match domain {
                Domain::Binary => {
                    for state in &["false", "true"] {
                        let _ = write_text_element_quiet(&mut writer, "OUTCOME", state);
                    }
                }
                Domain::Discrete { .. } => {
                    if let Some(states) = domain.states() {
                        for state in states {
                            let _ = write_text_element_quiet(&mut writer, "OUTCOME", state);
                        }
                    }
                }
                Domain::Continuous { .. } => {}
            }

            let _ = writer
                .write_event(Event::End(BytesEnd::new("VARIABLE")));
        }

        let order =
            network.topological_order().unwrap_or_else(|_| node_names.iter().map(|s| *s).collect());
        for var_name in &order {
            let cpt = match network.cpd(var_name) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let parent_ids = cpt.parent_ids();
            let parent_names: Vec<String> = parent_ids
                .iter()
                .filter_map(|pid| variables.get(pid).map(|v| v.name().to_string()))
                .collect();
            let factor = cpt.as_factor();
            let n = factor.scope().num_entries();
            let values: Vec<f64> = (0..n).map(|i| factor.value_at(i)).collect();

            let _ = writer
                .write_event(Event::Start(BytesStart::new("DEFINITION")));
            let _ = write_text_element_quiet(&mut writer, "FOR", var_name);

            for pn in &parent_names {
                let _ = write_text_element_quiet(&mut writer, "GIVEN", pn);
            }

            let table_str: String = values
                .iter()
                .map(|v| format!("{:.10}", v))
                .collect::<Vec<_>>()
                .join(" ");
            let _ = write_text_element_quiet(&mut writer, "TABLE", &table_str);

            let _ = writer
                .write_event(Event::End(BytesEnd::new("DEFINITION")));
        }

        let _ = writer
            .write_event(Event::End(BytesEnd::new("NETWORK")));
        let _ = writer
            .write_event(Event::End(BytesEnd::new("BIF")));

        String::from_utf8(writer.into_inner()).map_err(|e| LutufiError::SerializationError {
            reason: format!("UTF-8 encoding error: {}", e),
        })
    }

    /// Exports a Bayesian network to an XMLBIF file at the given path.
    pub fn export_to_file<P: AsRef<Path>>(network: &BayesianNetwork, path: P) -> LutufiResult<()> {
        let content = Self::export(network)?;
        std::fs::write(path.as_ref(), &content).map_err(|e| {
            LutufiError::SerializationError {
                reason: format!("Failed to write XMLBIF file: {}", e),
            }
        })?;
        Ok(())
    }

    /// Imports a Bayesian network from an XMLBIF-formatted string.
    pub fn import(content: &str) -> LutufiResult<BayesianNetwork> {
        let mut reader = Reader::from_str(content);
        let mut buf = Vec::new();

        let mut network = BayesianNetwork::new();
        let mut in_variable = false;
        let mut in_definition = false;
        let mut current_var_name = String::new();
        let mut current_var_type = String::new();
        let mut current_var_states: Vec<String> = Vec::new();
        let mut var_infos: Vec<(String, Vec<String>, bool)> = Vec::new();
        let mut def_for = String::new();
        let mut def_given: Vec<String> = Vec::new();
        let mut def_table: Vec<f64> = Vec::new();
        let mut definitions: Vec<(String, Vec<String>, Vec<f64>)> = Vec::new();
        let mut text_buf = String::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match tag.as_str() {
                        "VARIABLE" => {
                            in_variable = true;
                            current_var_states.clear();
                            current_var_name.clear();
                            let mut attr_type = "discrete".to_string();
                            for attr in e.attributes() {
                                if let Ok(a) = attr {
                                    if a.key.as_ref() == b"TYPE" {
                                        if let Ok(s) = std::str::from_utf8(&a.value) {
                                            attr_type = s.to_string();
                                        }
                                    }
                                }
                            }
                            current_var_type = attr_type;
                        }
                        "DEFINITION" => {
                            in_definition = true;
                            def_for.clear();
                            def_given.clear();
                            def_table.clear();
                        }
                        _ => {}
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match tag.as_str() {
                        "VARIABLE" => {
                            let mut attr_type = "discrete".to_string();
                            for attr in e.attributes() {
                                if let Ok(a) = attr {
                                    if a.key.as_ref() == b"TYPE" {
                                        if let Ok(s) = std::str::from_utf8(&a.value) {
                                            attr_type = s.to_string();
                                        }
                                    }
                                }
                            }
                            var_infos.push((
                                String::new(),
                                Vec::new(),
                                attr_type == "continuous",
                            ));
                        }
                        _ => {}
                    }
                }
                Ok(Event::Text(ref e)) => {
                    text_buf = e.unescape().unwrap_or_default().to_string();
                }
                Ok(Event::End(ref e)) => {
                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    match tag.as_str() {
                        "NAME" if in_variable => {
                            current_var_name = text_buf.clone();
                        }
                        "OUTCOME" if in_variable => {
                            current_var_states.push(text_buf.clone());
                        }
                        "VARIABLE" => {
                            if in_variable {
                                var_infos.push((
                                    current_var_name.clone(),
                                    current_var_states.clone(),
                                    current_var_type == "continuous",
                                ));
                                in_variable = false;
                            }
                        }
                        "FOR" if in_definition => {
                            def_for = text_buf.clone();
                        }
                        "GIVEN" if in_definition => {
                            def_given.push(text_buf.clone());
                        }
                        "TABLE" if in_definition => {
                            def_table = text_buf
                                .split(|c: char| c.is_whitespace())
                                .filter_map(|s| {
                                    if s.is_empty() {
                                        None
                                    } else {
                                        s.parse::<f64>().ok()
                                    }
                                })
                                .collect();
                        }
                        "DEFINITION" => {
                            if in_definition && !def_for.is_empty() && !def_table.is_empty() {
                                definitions.push((
                                    def_for.clone(),
                                    def_given.clone(),
                                    def_table.clone(),
                                ));
                            }
                            in_definition = false;
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => {
                    return Err(LutufiError::DeserializationError {
                        reason: format!("XML parse error: {}", e),
                    });
                }
                _ => {}
            }
            buf.clear();
        }

        for (var_name, states, is_cont) in &var_infos {
            if *is_cont {
                network.add_variable(var_name, Domain::continuous(None, None))?;
            } else if states.len() == 2
                && states.iter().any(|s| s == "false" || s == "0")
                && states.iter().any(|s| s == "true" || s == "1")
            {
                network.add_variable(var_name, Domain::binary())?;
            } else {
                network.add_variable(var_name, Domain::discrete(states.clone())?)?;
            }
        }

        for (child, parents, table) in &definitions {
            if network.id_of(child).is_err() {
                continue;
            }
            for parent in parents {
                if network.id_of(parent).is_ok() && network.id_of(child).is_ok() {
                    if !network
                        .edges()
                        .iter()
                        .any(|(f, t)| f == parent && t == child)
                    {
                        let _ = network.add_edge(parent, child);
                    }
                }
            }
            let child_var = network.variable(child)?.clone();
            let parent_vars: Vec<&Variable> = parents
                .iter()
                .filter_map(|pn| network.variable(pn).ok())
                .collect();

            if table.is_empty() {
                continue;
            }

            let child_domain_size = child_var.domain().size().unwrap_or(1);
            let parent_card: usize = parent_vars
                .iter()
                .map(|v| v.domain().size().unwrap_or(1))
                .product();
            let parent_card = parent_card.max(1);

            let mut matrix = vec![vec![0.0f64; parent_card]; child_domain_size];
            for pc in 0..parent_card {
                for cs in 0..child_domain_size {
                    let idx = pc * child_domain_size + cs;
                    if idx < table.len() {
                        matrix[cs][pc] = table[idx];
                    }
                }
            }

            if let Ok(cpt) =
                ConditionalProbabilityTable::from_values(&child_var, &parent_vars, matrix)
            {
                network.set_cpd(child, cpt).ok();
            }
        }

        Ok(network)
    }

    /// Imports a Bayesian network from an XMLBIF file at the given path.
    pub fn import_from_file<P: AsRef<Path>>(path: P) -> LutufiResult<BayesianNetwork> {
        let content = std::fs::read_to_string(path.as_ref()).map_err(|e| {
            LutufiError::DeserializationError {
                reason: format!("Failed to read XMLBIF file: {}", e),
            }
        })?;
        Self::import(&content)
    }
}

fn write_text_element_quiet(
    writer: &mut Writer<Vec<u8>>,
    tag: &str,
    content: &str,
) -> Result<(), quick_xml::Error> {
    writer.write_event(Event::Start(BytesStart::new(tag)))?;
    writer.write_event(Event::Text(BytesText::new(content)))?;
    writer.write_event(Event::End(BytesEnd::new(tag)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::domain::Domain;

    fn create_simple_network() -> BayesianNetwork {
        let mut net = BayesianNetwork::new();
        net.add_variable("X", Domain::binary()).unwrap();
        net.add_variable("Y", Domain::binary()).unwrap();
        net.add_edge("X", "Y").unwrap();
        let x_var = net.variable("X").unwrap().clone();
        let y_var = net.variable("Y").unwrap().clone();
        let cpt_x = ConditionalProbabilityTable::from_values(
            &x_var,
            &[] as &[&Variable],
            vec![vec![0.5], vec![0.5]],
        )
        .unwrap();
        net.set_cpd("X", cpt_x).unwrap();
        let cpt_y = ConditionalProbabilityTable::from_values(
            &y_var,
            &[&x_var],
            vec![vec![0.9, 0.2], vec![0.1, 0.8]],
        )
        .unwrap();
        net.set_cpd("Y", cpt_y).unwrap();
        net
    }

    #[test]
    fn test_xmlbif_export_contains_variables() {
        let net = create_simple_network();
        let xml = XmlBifFormat::export(&net).unwrap();
        assert!(xml.contains("<NAME>X</NAME>"));
        assert!(xml.contains("<NAME>Y</NAME>"));
        assert!(xml.contains("<FOR>X</FOR>"));
        assert!(xml.contains("<FOR>Y</FOR>"));
    }

    #[test]
    fn test_xmlbif_roundtrip() {
        let original = create_simple_network();
        let xml = XmlBifFormat::export(&original).unwrap();
        let imported = XmlBifFormat::import(&xml).unwrap();
        assert_eq!(original.nodes().len(), imported.nodes().len());
        assert_eq!(original.edges().len(), imported.edges().len());
        for name in original.nodes() {
            let orig_cpd = original.cpd(name).unwrap();
            let imp_cpd = imported.cpd(name).unwrap();
            let orig_f = orig_cpd.as_factor();
            let imp_f = imp_cpd.as_factor();
            let n = orig_f.scope().num_entries();
            let mut orig_vals: Vec<f64> = (0..n).map(|i| orig_f.value_at(i)).collect();
            let mut imp_vals: Vec<f64> = (0..n).map(|i| imp_f.value_at(i)).collect();
            orig_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
            imp_vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
            for i in 0..n {
                let diff = (orig_vals[i] - imp_vals[i]).abs();
                assert!(
                    diff < 1e-6,
                    "Mismatch at {}[{}]: {} vs {}",
                    name,
                    i,
                    orig_vals[i],
                    imp_vals[i]
                );
            }
        }
    }

    #[test]
    fn test_xmlbif_export_file() {
        let net = create_simple_network();
        let tmp = std::env::temp_dir().join("test_export.xmlbif");
        XmlBifFormat::export_to_file(&net, &tmp).unwrap();
        let imported = XmlBifFormat::import_from_file(&tmp).unwrap();
        let _ = std::fs::remove_file(&tmp);
        assert_eq!(imported.nodes().len(), 2);
    }
}
