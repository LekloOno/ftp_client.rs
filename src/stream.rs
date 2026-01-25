pub trait FtpStream {
    fn data_read(&mut self) -> Option<&mut dyn std::io::Read>;
    fn data_write(&mut self) -> Option<&mut dyn std::io::Write>;
    
    fn control_read(&mut self) -> &mut dyn std::io::Read;
    fn control_write(&mut self) -> &mut dyn std::io::Write;
}