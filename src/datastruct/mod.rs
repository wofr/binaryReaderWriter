use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::borrow::BorrowMut;

pub struct DataStruct {
    pub data_set1: i16,
    pub data_set2: i32,
    pub data_set3: i32,
    pub data_set4: i16,
    pub data_set5: i16,
    pub data_set6: i16,
    pub data_set7: i16,
    pub data_set8: i16,
    pub  data_set9: i16,
    pub data_set10: i32,
    pub data_set11: i16,
    pub data_set12: i16,
    pub data_set13: i16,
    pub data_set14: i32,
    pub data_set15: i32,
    pub data_set16: i32,
    pub  data_set17: i32,
    pub data_set18: i32,
    pub data_set19: i32,
    pub data_set20: i16,
    pub data_set21: i16,
    pub data_set22: i16,
    pub data_set23: i16,
    pub data_set24: i32,
    pub data_set25: i16,
    pub data_set26: i16,
    pub data_set27: i16,
    pub data_set28: i16,
    pub data_set29: Vec<u8>,
}

pub fn write_datastruct_to_file(filepath: &str, ds: &DataStruct) -> Result<usize,io::Error> {
    let mut f = File::create(filepath)?;
    let mut written_bytes = 0;

    written_bytes += f.write(&i16::to_le_bytes(ds.data_set1))?;
    written_bytes += f.write(&i32::to_le_bytes(ds.data_set2))?;
    written_bytes += f.write(&i32::to_le_bytes(ds.data_set3))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set4))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set5))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set6))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set7))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set8))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set9))?;
    written_bytes += f.write(&i32::to_le_bytes(ds.data_set10))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set11))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set12))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set13))?;
    written_bytes += f.write(&i32::to_le_bytes(ds.data_set14))?;
    written_bytes += f.write(&i32::to_le_bytes(ds.data_set15))?;
    written_bytes += f.write(&i32::to_le_bytes(ds.data_set16))?;
    written_bytes += f.write(&i32::to_le_bytes(ds.data_set17))?;
    written_bytes += f.write(&i32::to_le_bytes(ds.data_set18))?;
    written_bytes += f.write(&i32::to_le_bytes(ds.data_set19))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set20))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set21))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set22))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set23))?;
    written_bytes += f.write(&i32::to_le_bytes(ds.data_set24))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set25))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set26))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set27))?;
    written_bytes += f.write(&i16::to_le_bytes(ds.data_set28))?;
    written_bytes += f.write(ds.data_set29.as_slice())?;

    // f.write(i16::to_le_bytes(ds.data_set27));

    Ok(written_bytes)
}


/*
fn read_bytes < R: Read,  N: usize > (f: & mut R, const size:u8) -> [u8; size]{
    let mut bytes2 = [0u8; size];
    f.read( & mut bytes2);
    bytes2
}*/

pub fn read_datastruct_from_file(filepath: &str) -> Option<DataStruct> {
    let mut f = File::open(filepath).unwrap();

    let mut read_bytes2 =  |mut f:&File|{
        let mut bytes2 = [0u8; 2];
        f.read(&mut bytes2);
        bytes2
    };

    let  mut read_bytes4 =  |mut f:&File|{
        let mut bytes4 = [0u8; 4];
        f.read(&mut bytes4);
        bytes4
    };

    let read_rest =  |mut f:&File|{
        let mut bytesVec: Vec<u8> = vec![];
        f.read_to_end(&mut bytesVec);
        bytesVec
    };

   Some(DataStruct {
        data_set1:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set2:i32::from_le_bytes(read_bytes4(f.borrow_mut())),
        data_set3:i32::from_le_bytes(read_bytes4(f.borrow_mut())),
        data_set4:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set5:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set6:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set7:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set8:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set9:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set10:i32::from_le_bytes(read_bytes4(f.borrow_mut())),
        data_set11:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set12:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set13:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set14:i32::from_le_bytes(read_bytes4(f.borrow_mut())),
        data_set15:i32::from_le_bytes(read_bytes4(f.borrow_mut())),
        data_set16:i32::from_le_bytes(read_bytes4(f.borrow_mut())),
        data_set17:i32::from_le_bytes(read_bytes4(f.borrow_mut())),
        data_set18:i32::from_le_bytes(read_bytes4(f.borrow_mut())),
        data_set19:i32::from_le_bytes(read_bytes4(f.borrow_mut())),
        data_set20:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set21:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set22:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set23:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set24:i32::from_le_bytes(read_bytes4(f.borrow_mut())),
        data_set25:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set26:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set27:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set28:i16::from_le_bytes(read_bytes2(f.borrow_mut())),
        data_set29: read_rest(f.borrow_mut()),
    })





}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_datastruct_from_file(){
        let filePath ="DataStructFile2.bin";
        let bin =  read_datastruct_from_file(filePath);
        assert_eq!(true,bin.is_some());
        let bin = bin.unwrap();
        assert_eq!(3,bin.data_set1);
        assert_eq!(164,bin.data_set2);
        assert_eq!(5501,bin.data_set3);
        assert_eq!(601,bin.data_set4);
        assert_eq!(1,bin.data_set5);
        assert_eq!(0,bin.data_set6);
        assert_eq!(1,bin.data_set7);
        assert_eq!(3,bin.data_set8);
        assert_eq!(0,bin.data_set9);
        assert_eq!(255,bin.data_set10);
        assert_eq!(0,bin.data_set11);
        assert_eq!(1,bin.data_set12);
        assert_eq!(0,bin.data_set13);
        assert_eq!(1404216000,bin.data_set14);
        assert_eq!(1404216000,bin.data_set15);
        assert_eq!(1404219600,bin.data_set16);
        assert_eq!(1404219600,bin.data_set17);
        assert_eq!(1,bin.data_set18);
        assert_eq!(1,bin.data_set19);
        assert_eq!(0,bin.data_set20);
        assert_eq!(1,bin.data_set21);
        assert_eq!(1,bin.data_set22);
        assert_eq!(0,bin.data_set23);
        assert_eq!(88,bin.data_set24);
        assert_eq!(0,bin.data_set25);
        assert_eq!(1,bin.data_set26);
        assert_eq!(0,bin.data_set27);
        assert_eq!(0,bin.data_set28);
    }

    #[test]
    fn test_datastruct_to_file_writer() {
        let filePath = "Test.bin";
        let occ = vec![1,2,4];
        let occLen = occ.len();
        let bin = DataStruct {
            data_set1: 0,
            data_set2: 0,
            data_set3: 0,
            data_set4: 0,
            data_set5: 0,
            data_set6: 0,
            data_set7: 0,
            data_set8: 0,
            data_set9: 0,
            data_set10: 0,
            data_set11: 0,
            data_set12: 0,
            data_set13: 0,
            data_set14: 0,
            data_set15: 0,
            data_set16: 0,
            data_set17: 0,
            data_set18: 0,
            data_set19: 0,
            data_set20: 0,
            data_set21: 0,
            data_set22: 0,
            data_set23: 0,
            data_set24: 0,
            data_set25: 0,
            data_set26: 0,
            data_set27: 0,
            data_set28: 0,
            data_set29: occ,
        };
        assert_eq!(76+occLen, write_datastruct_to_file(filePath, &bin).unwrap());
    }
}