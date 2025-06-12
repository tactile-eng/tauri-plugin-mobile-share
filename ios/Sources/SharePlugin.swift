import SwiftRs
import Tauri
import UIKit
import WebKit

class ShareMeta: Decodable {
    let name: String
    let ext: String
}

class ShareText: Decodable {
    let text: String
    let meta: ShareMeta
}

class ShareBinary: Decodable {
    let data: Data
    let meta: ShareMeta
}

class SharePlugin: Plugin {
    @objc public func shareText(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(ShareText.self)
        let url = storeTempWithUrl(args.meta)

        try? args.text.write(to: url, atomically: true, encoding: .utf8)

        baseShare(invoke, [url])
    }

    @objc public func shareBinary(_ invoke: Invoke) throws {
        let args = try invoke.parseArgs(ShareBinary.self)
        let url = storeTempWithUrl(args.meta)

        try? args.data.write(to: url, options: .atomic)

        baseShare(invoke, [url])
    }

    func storeTempWithUrl(_ meta: ShareMeta) -> URL {
        return FileManager.default.temporaryDirectory
            .appendingPathComponent(meta.name)
            .appendingPathExtension(meta.ext)
    }

    func baseShare(_ invoke: Invoke, _ items: [Any]) {
        let response = { () -> Void in invoke.resolve(true) }
        let reject = { () -> Void in invoke.reject("could not access main ViewController") }

        DispatchQueue.main.async {
            guard let viewController = self.manager.viewController else {
                reject()
                return
            }

            viewController.present(
                UIActivityViewController(
                    activityItems: items,
                    applicationActivities: nil
                ),
                animated: true,
                completion: { () -> Void in response() }
            )
        }

        // await withCheckedContinuation { continuation in
        //     dispatch(items) { res in continuation.resume(returning: res) }
        // } ? response() : reject()
    }

    // func dispatch(_ items: [Any], completion: @escaping (Bool) -> Void) {
    //     DispatchQueue.main.async {
    //         guard let viewController = self.manager.viewController else {
    //             completion(false)
    //             return
    //         }

    //         viewController.present(
    //             UIActivityViewController(
    //                 activityItems: items,
    //                 applicationActivities: nil
    //             ),
    //             animated: true,
    //             completion: { () -> Void in completion(true) }
    //         )
    //     }
    // }
}

@_cdecl("init_plugin_test")
func initPlugin() -> Plugin {
    return SharePlugin()
}
