import init, { parse, setup_context, setup, decode_textures, has_astc, has_bptc } from './inochi2d.js';

let gl;
let renderer;
let data;

let model = window.sessionStorage.getItem('model');
if (!model) {
  model = 'aka';
  window.sessionStorage.setItem('model', model);
}

let format = window.sessionStorage.getItem('format');
if (!format) {
  format = 'tga';
  window.sessionStorage.setItem('format', format);
}

async function fetch_array(url) {
  const response = await fetch(url);
  const buffer = await response.arrayBuffer();
  return new Uint8Array(buffer);
}

function get_name() {
  return model + '.' + format + '.inp';
}

async function run() {
  await init();
  const array = await fetch_array(get_name());

  gl = setup_context('canvas');
  document.getElementById('astc').disabled = !has_astc(gl);
  document.getElementById('bc7').disabled = !has_bptc(gl);

  change_model()
}

async function change_model() {
  const array = await fetch_array(get_name());
  data = parse(array);
  const textures = decode_textures(data);
  setup(gl, data, textures);
}

const buttons = document.getElementsByTagName('button');
for (let button of buttons) {
  button.addEventListener('click', async function(evt) {
    model = evt.target.id;
    window.sessionStorage.setItem('model', model);
    await change_model();
  });
}

const inputs = document.getElementsByTagName('input');
for (let input of inputs) {
  if (input.id == format) {
    input.checked = true;
  }
  input.addEventListener('change', async function(evt) {
    format = evt.target.id;
    window.sessionStorage.setItem('format', format );
    await change_model();
  });
}

run();