pub enum PrgfError <'a> {
    ft_not_found(&'a str),
    ft_file_null(&'a str),
}