// swift-tools-version: 5.8
import PackageDescription

let package = Package(
    name: "App",
    products: [
        .library(
            name: "App",
            targets: ["App"]
        )
    ],
    targets: [
        .target(
            name: "App",
            dependencies: ["Serde"]
        ),
        .target(
            name: "Serde",
            dependencies: []
        ),
    ]
)
