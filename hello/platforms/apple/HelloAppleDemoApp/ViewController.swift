// 

import UIKit
import Hello

class ViewController: UIViewController {

  override func viewDidLoad() {
    super.viewDidLoad()
    // Do any additional setup after loading the view.
//    print(rustGreeting(to: "Bob"))
    print(Hello.add(a: 1, b: 2))
  }
}

