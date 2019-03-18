extern crate protobuf;
use std::fs::File;
use protobuf::{parse_from_reader, ProtobufResult};
use serde::{Serialize, Deserialize};

#[macro_use]
extern crate tera;

mod proto;

#[derive(Serialize, Deserialize, Debug)]
struct PrintableMethod {
    Name: String,
    InputType: String,
    OutputType: String,
}

fn main() {
    let mut file = match File::open(&"helloworld.binaryproto") {
        Err(why) => panic!("couldn't open"),
        Ok(file) => file,
    };

   let mut fs = match parse_from_reader::<proto::descriptor::FileDescriptorSet>(&mut file) {
       Err(why) => panic!("error"),
       Ok(file) => file
   };
   
   
  let tera = match tera::Tera::new("src/templates/**/*") {
       Err(why) => panic!("error reading templates"),
       Ok(tera) => tera
   };
   

   for file in fs.get_file() { 
       for service in file.get_service() {
           let mut ctx = tera::Context::new();
           ctx.add("packageName", file.get_package());
           ctx.add("serviceName", service.get_name());
           let printableMethods: Vec<PrintableMethod> = service.get_method().iter()
                .map(|method| PrintableMethod{
                    Name: method.get_name().to_string(),
                    InputType: rust_type_name_from_proto_name(method.get_input_type().to_string()),
                    OutputType: rust_type_name_from_proto_name(method.get_output_type().to_string()),
                }).collect();
           ctx.add("methods", &printableMethods);
           let rendered = tera.render("node.rs.tmpl", &ctx).expect("Failed to render template");
           println!("{}", rendered);
      }
   }
   
    
}

fn rust_type_name_from_proto_name(proto_type_name: String) -> String {
    let pieces : Vec<&str> = proto_type_name.rsplit(".").collect();
    format!("proto::{}::{}", pieces[1], pieces[0])
}
