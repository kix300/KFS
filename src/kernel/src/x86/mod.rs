pub mod gdt;


#[repr(C, packed)]
pub struct DescriptorTable {
	/// Size of the table in bytes, minus 1.
	size: u16,
	/// Linear address of the table (not physical address, paging applies).
	offset: u32,
}
