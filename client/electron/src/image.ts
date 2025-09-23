// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import sharp from 'sharp';

function makeNightly(image: sharp.Sharp): void {
  image.linear(1.1, -15).tint('#ff9933');
}

function makeDev(image: sharp.Sharp): void {
  image.grayscale(true);
}

function makeRC(image: sharp.Sharp): void {
  image.modulate({ saturation: 0.5, lightness: 0.5 });
}

export const ImageFilters = {
  makeNightly,
  makeDev,
  makeRC,
};
