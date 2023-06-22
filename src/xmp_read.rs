use std::env;

use anyhow::{anyhow, Context, Result};
use xmp_toolkit::{OpenFileOptions, XmpFile, XmpMeta, XmpValue};

pub(crate) fn xmp_read() -> Result<()> {
    // Parse command-line arguments. There should be only one
    // argument: a path to a file to be read.
    let args: Vec<String> = env::args().collect();

    let path = match args.len() {
        // args[0] = path to executable
        2 => Ok(&args[1]),
        n => Err(anyhow!(
            "expected 1 argument (file name), got {} arguments",
            n - 1
        )),
    }?;
    // Open the file for read-only access and request to use a format-specific
    // handler.
    let mut f = XmpFile::new()?;

    f.open_file(
        path,
        OpenFileOptions::default()
            .for_update()
            .only_xmp()
            .use_smart_handler(),
    )
    .or_else(|_err| {
        // There might not be an appropriate handler available.
        // Retry using packet scanning, providing a different set of
        // open-file options.
        eprintln!(
            "No smart handler available for file {}. Trying packet scanning.",
            path
        );
        f.open_file(path, OpenFileOptions::default().use_packet_scanning())
    })
    .with_context(|| format!("could not find XMP in file {}", path))?;

    // Retrieve the XMP from the file.
    let mut xmp = f
        .xmp()
        .ok_or_else(|| anyhow!("unable to process XMP in file {}", path))?;

    let xmp_dm_uri = "http://ns.adobe.com/xmp/1.0/DynamicMedia/".to_string();
    let xmp_dm = XmpMeta::register_namespace(xmp_dm_uri.as_str(), "xmpDM")?;

    if xmp.contains_property(xmp_dm_uri.as_str(), "tempo") {
        if let Some(value) = xmp.property(xmp_dm_uri.as_str(), "tempo") {
            println!("meta:MetadataTempo = {}", value.value);
        }
    } else {
        println!("could not find {}{}", xmp_dm, "tempo");
    }
    if f.can_put_xmp(&xmp) {
        match xmp.set_property(
            xmp_dm_uri.as_str(),
            "tempo",
            &XmpValue::new("33".to_string()),
        ) {
            Ok(_) => println!("PRINTED TO TEMPO!"),
            Err(_) => println!("could not write new value to tempo"),
        };
        match xmp.set_property_bool(xmp_dm_uri.as_str(), "loop", &XmpValue::new(true)) {
            Ok(_) => println!("PRINTED TO boolean LOOP!"),
            Err(_) => println!("could not write new value to loop"),
        };
        f.put_xmp(&xmp).unwrap();
        f.close();
    } else {
        println!("can't update file");
    }

    Ok(())
}
