use pqc_kyber as kyber;
use pqc_dilithium as dilithium;
use pqc_sphincsplus as sphincs;

//Funcion de prueba de kyber
fn prueba_kyber() -> Result<(u128,u128), kyber::KyberError>{
    let mut rng = rand::thread_rng();
    let llaves_usuarioa = kyber::keypair(&mut rng)?;
    
    //Medicion del tiempo de encapsulamiento
    let cronometro = std::time::Instant::now();
    let (texto_cifrado, secreto_compartido_usuariob) = kyber::encapsulate(&llaves_usuarioa.public, &mut rng)?;
    let tiempo_encapsula = cronometro.elapsed().as_micros();
    
    //Medicion tiempo de desencapsulamiento
    let cronometro = std::time::Instant::now();
    let secreto_compartido_usuarioa = kyber::decapsulate(&texto_cifrado, &llaves_usuarioa.secret)?;
    let tiempo_desencapsula = cronometro.elapsed().as_micros();

    if secreto_compartido_usuariob == secreto_compartido_usuarioa {
        return Ok((tiempo_encapsula, tiempo_desencapsula));
    }
    else{ 
        return Err(kyber::KyberError::Decapsulation);
    }
}

//Funcion que comprueba el firmado por dilithium.
fn prueba_dilithium(mensaje: &[u8]) -> Result<(u128,u128), dilithium::SignError>{
    let llaves = dilithium::Keypair::generate();
    
    //Tiempo de firma del archivo
    let cronometro = std::time::Instant::now();
    let firma = llaves.sign(&mensaje);
    let tiempo_firma = cronometro.elapsed().as_micros();
    
    //Tiempo de verificacion del archivo
    let cronometro = std::time::Instant::now();
    let verifica = dilithium::verify(&firma, &mensaje, &llaves.public);
    let tiempo_verifica = cronometro.elapsed().as_micros();

    if verifica.is_ok(){
        return Ok((tiempo_firma, tiempo_verifica));
    }else{
        return Err(dilithium::SignError::Verify);
    }
}

fn prueba_sphincs(mensaje: &[u8]) -> Result<(u128, u128), sphincs::SigError>{
    let llaves = sphincs::keypair();

    //Tiempo de firma del archivo
    let cronometro = std::time::Instant::now();
    let firma = sphincs::sign(&mensaje,&llaves);
    let tiempo_firma = cronometro.elapsed().as_micros();

    //Tiempo de verificacion del archivo
    let cronometro = std::time::Instant::now();
    let verifica = sphincs::verify(&firma,&mensaje,&llaves);
    let tiempo_verifica = cronometro.elapsed().as_micros();

    if verifica.is_ok(){
        return Ok((tiempo_firma,tiempo_verifica));
    }else{
        return Err(sphincs::SigError::Verify);
    }
}

//Funcion principal de ejecucion.
fn main(){
    println!("Ejecucion Kyber:");
    let nombre_archivo = "1024KB.txt";
    let archivo = std::fs::read_to_string(nombre_archivo).expect("No se pudo abrir el documento");
    let archivo = archivo.as_bytes();

    //Ejecucucion de Kyber
    match prueba_kyber(){
        Ok((encapsula,desencapsula)) => println!("Kyber - Encapsula:{} Desencapsula:{}", encapsula, desencapsula),
        Err(error) => println!("Kyber: ejecucion con errores: {}", error),
    }

    //Ejecucion de Dilithium
    match prueba_dilithium(archivo){
        Ok((firma,verifica)) => println!("Dilithium - Firma:{}  Verifica:{}", firma, verifica),
        Err(_error) => println!("Dilithium: ejecucion con errores"),
    }

    //Ejecucion de Sphincs
    match prueba_sphincs(archivo){
        Ok((firma,verifica)) => println!("Sphincs - Firma:{} Verifica:{}",firma,verifica),
        Err(_error) => println!("Sphincs: ejecucion con errores"),
    }
}
