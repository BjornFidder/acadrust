use acadrust::entities::{AttributeDefinition, AttributeEntity, Insert};
use acadrust::{
    CadDocument, DwgReader, DwgWriter, DxfReader, DxfVersion, DxfWriter, EntityType, Vector3,
};
use std::io::Cursor;

#[test]
fn signed_attribute_thickness_survives_physical_readback() {
    let mut doc = CadDocument::with_version(DxfVersion::AC1032);
    let mut definition = AttributeDefinition::simple("DEPTH");
    definition.thickness = -3.25;
    let def = doc
        .add_entity(EntityType::AttributeDefinition(definition))
        .unwrap();
    let mut value = AttributeEntity::simple("DEPTH", "A01");
    value.thickness = 2.5;
    let mut insert = Insert::new("*Model_Space", Vector3::ZERO);
    insert.attributes.push(value);
    let ins = doc.add_entity(EntityType::Insert(insert)).unwrap();
    let dxf = DxfWriter::new(&doc).write_to_vec().unwrap();
    let dwg = DwgWriter::write_to_vec(&doc).unwrap();
    for loaded in [
        DxfReader::from_reader(Cursor::new(dxf))
            .unwrap()
            .read()
            .unwrap(),
        DwgReader::from_stream(Cursor::new(dwg)).read().unwrap(),
    ] {
        let EntityType::AttributeDefinition(d) = loaded.get_entity(def).unwrap() else {
            panic!("definition missing")
        };
        assert_eq!(d.thickness, -3.25);
        let EntityType::Insert(i) = loaded.get_entity(ins).unwrap() else {
            panic!("insert missing")
        };
        assert_eq!(i.attributes.len(), 1);
        assert_eq!(i.attributes[0].thickness, 2.5);
        assert_eq!(i.attributes[0].value, "A01");
    }
}
