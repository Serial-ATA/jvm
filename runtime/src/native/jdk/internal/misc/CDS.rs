use crate::native::NativeReturn;
use crate::stack::local_stack::LocalStack;

use common::int_types::s4;
use instructions::Operand;

pub fn isDumpingClassList0(_: LocalStack) -> NativeReturn {
	Some(Operand::Int(s4::from(false)))
}
pub fn isDumpingArchive0(_: LocalStack) -> NativeReturn {
	Some(Operand::Int(s4::from(false)))
}
pub fn isSharingEnabled0(_: LocalStack) -> NativeReturn {
	Some(Operand::Int(s4::from(false)))
}
pub fn logLambdaFormInvoker(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.CDS#logLambdaFormInvoker")
}

pub fn initializeFromArchive(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.CDS#initializeFromArchive")
}
pub fn defineArchivedModules(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.CDS#defineArchivedModules")
}
pub fn getRandomSeedForDumping(_: LocalStack) -> NativeReturn {
	// TODO: https://github.com/openjdk/jdk/blob/af564e46b006fcd57ec7391cd1438b3b9311b1d6/src/hotspot/share/prims/jvm.cpp#L3696
	Some(Operand::Long(0))
}

pub fn dumpClassList(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.CDS#dumpClassList")
}
pub fn dumpDynamicArchive(_: LocalStack) -> NativeReturn {
	unimplemented!("jdk.internal.misc.CDS#dumpDynamicArchive")
}
