// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "Shared",
    platforms: [
        .iOS(.v16),
        .macOS(.v13),
    ],
    products: [
        .library(
            name: "Shared",
            targets: ["Shared"]
        ),
    ],
    targets: [
        .binaryTarget(
            name: "SharedFFI",
            path: "Shared.xcframework"
        ),
        .target(
            name: "Shared",
            dependencies: ["SharedFFI"],
            path: "Sources"
        ),
    ]
)