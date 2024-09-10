/*
Fayl Shifrlovchi/Dekriplovchi
    Reja:
Maqsad: Faylni shifrlash va kerak bo'lganda uni dekriplash qilish.
Kutubxonalar: aes, rand, base64.
    Amallar:
1.Foydalanuvchidan fayl yo'lini va parolni olish.
2.Faylni AES algoritmi yordamida shifrlash.
3.Shifrlangan faylni saqlash.
4.Faylni dekriplash.
    Algoritm:
.Foydalanuvchi kiritgan faylni o'qing.
.Fayl mazmunini AES bilan shifrlang.
.Shifrlangan ma'lumotni faylga saqlang yoki foydalanuvchiga chiqaring.
.Dekriplash jarayonini amalga oshiring.
*/

use std::{error::Error, fs};
use aes::Aes128;
use cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};

fn main() -> Result<(), Box<dyn Error>> {
    
    let key = b"qwertyuiopasd123"; //128-bitli kalit 16 bayitli
    let data = fs::read("matn.txt")?; // fayil malumotlarini o'qish
    let shifrlash: Aes128 = Aes128::new(GenericArray::from_slice(key)); //shifirlash obyektini yaratish

    let mut block = GenericArray::clone_from_slice(&data[..16]); //har safar 16 bayt yaratadi
    shifrlash.encrypt_block(&mut block); //malumotlarni shifirlash
    fs::write("shifrtlangan.tex", &block)?; //shifirlangan malumotlarni  yangi fayilga yozish

    let mut ochilgan_shrifr = block.clone(); //shriftlangan blokni klonlash
    shifrlash.decrypt_block(&mut ochilgan_shrifr); //blokni qayta ochish
    println!("{:?}", ochilgan_shrifr);

    Ok(())
}
