use crate::array::{BoxedArrayBuilder, UTF8ArrayBuilder};
use crate::error::ErrorCode::InternalError;
use crate::error::{Result, RwError};
use crate::types::{DataType, DataTypeKind, DataTypeRef};
use risingwave_proto::data::{DataType as DataTypeProto, DataType_TypeName};
use std::any::Any;
use std::convert::TryFrom;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub struct StringType {
    nullable: bool,
    width: usize,
    kind: DataTypeKind,
}

impl DataType for StringType {
    fn data_type_kind(&self) -> DataTypeKind {
        self.kind
    }

    fn is_nullable(&self) -> bool {
        self.nullable
    }

    fn create_array_builder(self: Arc<Self>, capacity: usize) -> Result<BoxedArrayBuilder> {
        let width = self.width;
        UTF8ArrayBuilder::new(self, width, capacity)
            .map(|builder| Box::new(builder) as BoxedArrayBuilder)
    }

    fn to_protobuf(&self) -> Result<DataTypeProto> {
        let mut proto = DataTypeProto::new();

        proto.set_is_nullable(self.nullable);
        proto.set_precision(self.width as u32);

        match self.kind {
            DataTypeKind::Char => {
                proto.set_type_name(DataType_TypeName::CHAR);
                Ok(proto)
            }
            DataTypeKind::Varchar => {
                proto.set_type_name(DataType_TypeName::VARCHAR);
                Ok(proto)
            }
            _ => Err(InternalError(format!(
                "Incorrect data type kind for string type: {:?}",
                self.kind
            ))
            .into()),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl<'a> TryFrom<&'a DataTypeProto> for StringType {
    type Error = RwError;

    fn try_from(proto: &'a DataTypeProto) -> Result<Self> {
        match proto.get_type_name() {
            DataType_TypeName::CHAR => Ok(Self {
                nullable: proto.is_nullable,
                width: proto.precision as usize,
                kind: DataTypeKind::Char,
            }),
            DataType_TypeName::VARCHAR => Ok(Self {
                nullable: proto.is_nullable,
                width: proto.precision as usize,
                kind: DataTypeKind::Varchar,
            }),
            _ => Err(InternalError(format!(
                "Incorrect data type kind for string type: {:?}",
                proto.get_type_name()
            ))
            .into()),
        }
    }
}

impl StringType {
    pub fn create(nullable: bool, width: usize, kind: DataTypeKind) -> DataTypeRef {
        Arc::new(Self {
            nullable,
            width,
            kind,
        }) as DataTypeRef
    }
}
