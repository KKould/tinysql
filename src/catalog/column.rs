// 列定义
#[derive(Debug, Clone)]
pub struct Column {
    pub column_name: String,
    pub column_type: DataType,
    // 内联列则为固定列的大小，否则为指针大小
    pub fixed_len: usize,
    // 内联列则为0，否则为变长列的大小
    pub variable_len: usize,
    // 列在元组中的偏移量
    pub column_offset: usize,
}
impl Column {
    pub fn new(column_name: String, column_type: DataType, variable_len: usize) -> Self {
        Self {
            column_name,
            column_type,
            fixed_len: column_type.type_size(),
            variable_len,
            column_offset: 0,
        }
    }

    pub fn is_inlined(&self) -> bool {
        self.column_type != DataType::Varchar
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataType {
    Boolean,
    TinyInt,
    SmallInt,
    Integer,
    BigInt,
    Decimal,
    Varchar,
    Timestamp,
}
impl DataType {
    pub fn type_size(&self) -> usize {
        match self {
            DataType::Boolean => 1,
            DataType::TinyInt => 1,
            DataType::SmallInt => 2,
            DataType::Integer => 4,
            DataType::BigInt => 8,
            DataType::Decimal => 8,
            // TODO 指针大小，暂时跟bustub保持一致
            DataType::Varchar => 12,
            DataType::Timestamp => 8,
        }
    }
}
