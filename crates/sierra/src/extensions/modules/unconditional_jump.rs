use crate::extensions::lib_func::{
    LibFuncSignature, OutputBranchInfo, SierraApChange, SignatureOnlyConcreteLibFunc,
    SignatureSpecializationContext, SpecializationContext,
};
use crate::extensions::{NoGenericArgsGenericLibFunc, SpecializationError};
use crate::ids::GenericLibFuncId;

#[derive(Default)]
pub struct UnconditionalJumpLibFunc {}
impl NoGenericArgsGenericLibFunc for UnconditionalJumpLibFunc {
    type Concrete = SignatureOnlyConcreteLibFunc;
    const ID: GenericLibFuncId = GenericLibFuncId::new_inline("jump");

    fn specialize_signature(
        &self,
        _context: &dyn SignatureSpecializationContext,
    ) -> Result<LibFuncSignature, SpecializationError> {
        Ok(LibFuncSignature {
            input_types: vec![],
            output_info: vec![OutputBranchInfo { vars: vec![], ap_change: SierraApChange::Known }],
            fallthrough: None,
        })
    }

    fn specialize(
        &self,
        context: &dyn SpecializationContext,
    ) -> Result<Self::Concrete, SpecializationError> {
        Ok(SignatureOnlyConcreteLibFunc {
            signature: <Self as NoGenericArgsGenericLibFunc>::specialize_signature(
                self,
                context.upcast(),
            )?,
        })
    }
}
