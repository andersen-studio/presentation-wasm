use presentation_wasm::run;

fn main() {
    pollster::block_on(run());
}
