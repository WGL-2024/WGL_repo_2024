use crate::types::packet::Fragment;
struct Message {}

impl Message {
    fn serialize(&self) -> String {
        unimplemented!()
    }
    //takes message and returns the data struct serialized in a String
    //so it goes from the actual data struct to a String
    fn deserialize(serialized: String) -> Message {
        unimplemented!()
    }
    //Takes the content String and makes an instance of Message from it
    fn disassembly(serialized: String) -> Vec<Fragment> {
        unimplemented!()
    }
    //takes the String and splits it into Fragments
    fn assembly(fragments: Vec<Fragment>) -> String {
        unimplemented!()
    }
    //takes a bunch of Fragments and composes them in a serialized string.
}
