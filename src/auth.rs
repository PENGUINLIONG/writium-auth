use writium::prelude::*;

/// An authority is who recognizes a remote and decides whether it is capable of
/// accessing to certain resources and taking certain actions, using information
/// provided in its request sent.  
/// It is hightly recommended to let an authority to manage privilege for
/// unsafe HTTP methods, i.e., DELETE, PATCH and PUT.
/// 
/// # Authentication and Authorization
///
/// Authentication is the process the authority extract credential from a
/// request, match the credential with an corresponding identity, and map the
/// identity into an internal representation.
///
/// Authorization is the process the authority check whether the inquired
/// priviledge is available for an identity.
///
/// Authentication could be a part of the authorization process.
///
/// The separation is not forced here because the use of `future` as result,
/// because it is awful when we have to borrow `self` in future calls.
pub trait Authority: 'static + Send + Sync {
    /// A value denoting the privilege the system withholds. Generally, an enum
    /// or a string namespaced by dot (`.`) is used.
    type Privilege: 'static;
    /// Decides whether the identity is capable of being granted with the
    /// inquired privilege.  
    /// An implementation SHOULD use the mapped identity and check if the
    /// inquired privilege is available for it. A remote process can be
    /// involved.
    fn authorize(&self, pr: Self::Privilege, req: &Request) -> Result<()>; 
}
