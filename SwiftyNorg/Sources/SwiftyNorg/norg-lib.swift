import RustXcframework
public func get_length<GenericIntoRustString: IntoRustString>(_ term: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$get_length({ let rustString = term.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func read_file<GenericIntoRustString: IntoRustString>(_ url: GenericIntoRustString) -> RustString {
    RustString(ptr: __swift_bridge__$read_file({ let rustString = url.intoRustString(); rustString.isOwned = false; return rustString.ptr }()))
}
public func search<GenericToRustStr: ToRustStr>(_ path: GenericToRustStr, _ query: GenericToRustStr) -> RustVec<RustString> {
    return query.toRustStr({ queryAsRustStr in
        return path.toRustStr({ pathAsRustStr in
        RustVec(ptr: __swift_bridge__$search(pathAsRustStr, queryAsRustStr))
    })
    })
}


