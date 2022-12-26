// replace spaces with '_' and urlencode the remainder
function name_to_path(input) {
  let output = input.replace(/ /g, '_');
  console.log('name_to_path()');
  console.log(output);
  return encodeURI(output); // don't encode '/'
}

// url-decode and replace '_' with spaces
function path_to_name(input) {
  let output = decodeURIComponent(input); // seek to decode ALL (including %2f->'/')
  return output.replace(/_/g, ' ');
}

module.exports = { name_to_path, path_to_name };
