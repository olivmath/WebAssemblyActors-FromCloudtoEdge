let importObject = {};

WebAssembly.instantiateStreaming(
    fetch('./math.wasm'),
    importObject
).then(obj => {
    console.log(obj.instance.exports.add(2, 3))
    console.log(obj.instance.exports.sub(2, 3))
    console.log(obj.instance.exports.div(2, 3))
    console.log(obj.instance.exports.mul(2, 3))
});