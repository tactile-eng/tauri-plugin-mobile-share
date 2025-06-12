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
        let completionHandler = {
            (_: UIActivity.ActivityType?, completed: Bool, _: [Any]?, maybeError: (any Error)?)
                -> Void in
            if let error = maybeError {
                invoke.reject("failed to share data...")
            } else {
                invoke.resolve(completed)
            }
        }

        let activityViewController = UIActivityViewController(
            activityItems: items, applicationActivities: nil
        )

        activityViewController.completionWithItemsHandler = completionHandler

        DispatchQueue.main.async {
            guard let viewController = self.manager.viewController else {
                invoke.reject("could not access main ViewController")
                return
            }

            viewController.present(
                activityViewController, animated: true, completion: nil
            )
        }
    }
}

@_cdecl("init_plugin_test")
func initPlugin() -> Plugin {
    return SharePlugin()
}
