//const canvas = document.getElementById('canvas');
//const ctx = canvas.getContext('2d');

// First up, but try to do feature detection to provide better error messages
function loadWasm() {
  let msg = 'This demo requires a current version of Firefox (e.g., 70.0) with\n'
  msg += 'the `javascript.options.shared_memory` option enabled in `about:config`';
  if (typeof SharedArrayBuffer !== 'function') {
    alert('this browser does not have SharedArrayBuffer support enabled' + '\n\n' + msg);
    return
  }
  // Test for bulk memory operations with passive data segments
  //  (module (memory 1) (data passive ""))
  const buf = new Uint8Array([0x00, 0x61, 0x73, 0x6d, 0x01, 0x00, 0x00, 0x00,
    0x05, 0x03, 0x01, 0x00, 0x01, 0x0b, 0x03, 0x01, 0x01, 0x00]);
  if (!WebAssembly.validate(buf)) {
    alert('this browser does not support passive wasm memory, demo does not work' + '\n\n' + msg);
    return
  }

  wasm_bindgen('./shotcaller_bg.wasm')
    .then(run)
    .catch(console.error);
}

loadWasm();

const { run_test, Comp, WorkerPool } = wasm_bindgen;

function run() {
  // The maximal concurrency of our web worker pool is `hardwareConcurrency`,
  // so set that up here and this ideally is the only location we create web
  // workers.
  let pool = new WorkerPool(navigator.hardwareConcurrency);

  console.log('run start');
  run_test(navigator.hardwareConcurrency, pool);
  console.log('run stop');
}

