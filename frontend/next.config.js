// see https://github.com/openlayers/openlayers/issues/10470

const withTranspile = require('next-transpile-modules')(['rlayers']);
const path = require('path');

module.exports = withTranspile();
