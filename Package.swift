// swift-tools-version:5.5.0
import PackageDescription
let package = Package(
	name: "SwiftyNorg",
	products: [
		.library(
			name: "SwiftyNorg",
			targets: ["SwiftyNorg"]),
	],
	dependencies: [],
	targets: [
		.binaryTarget(
			name: "RustXcframework",
			path: "RustXcframework.xcframework"
		),
		.target(
			name: "SwiftyNorg",
			dependencies: ["RustXcframework"])
	]
)
	