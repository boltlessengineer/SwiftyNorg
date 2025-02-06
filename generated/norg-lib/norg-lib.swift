public func parse<GenericToRustStr: ToRustStr>(_ content: GenericToRustStr) -> RustString {
    return content.toRustStr({ contentAsRustStr in
        RustString(ptr: __swift_bridge__$parse(contentAsRustStr))
    })
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


