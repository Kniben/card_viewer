
window.addEventListener('load', async () => {
    // the `wasm_bindgen` global is set to the exports of the Rust module
    //
    // here we tell bindgen the path to the wasm file so it can run
    // initialization and return to us a promise when it's done
    // also, we can use 'await' on the returned promise
    await wasm_bindgen('./wasm_bindgen/card_viewer_bg.wasm');

    var state = wasm_bindgen.State.init();

    ["mousemove", "mousedown", "mouseup", "DOMMouseScroll"]
        .forEach(function(eventName) {
            document.addEventListener(eventName, function(event) {state.on_document_event(event)});
    });

	window.requestAnimationFrame(animateJs);

	function animateJs() {
	    state.animate();
	    window.requestAnimationFrame(animateJs);
	}
});
