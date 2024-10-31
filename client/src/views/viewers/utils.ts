// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import { EntryName, FsPath } from '@/parsec';

export interface FileContentInfo {
  data: Uint8Array;
  extension: string;
  mimeType: string;
  fileName: EntryName;
  path: FsPath;
}

export interface ImageViewerConfig {
  zoomLevel: number;
}

export type ViewerConfig = ImageViewerConfig;

export const ZOOM_LEVELS = [5, 10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 125, 150, 175, 200, 250, 300, 400, 500];

function setZoomLevel(config: ViewerConfig, zoomLevel: number): ViewerConfig {
  if (!ZOOM_LEVELS.includes(zoomLevel) || config.zoomLevel === zoomLevel) {
    return config;
  }
  return { ...config, zoomLevel };
}

function zoomOut(config: ViewerConfig): ViewerConfig {
  const currentIndex = ZOOM_LEVELS.indexOf(config.zoomLevel);
  if (currentIndex === 0) {
    return config;
  }
  return setZoomLevel(config, ZOOM_LEVELS[currentIndex - 1]);
}

function resetZoom(config: ViewerConfig): ViewerConfig {
  return setZoomLevel(config, 100);
}

function zoomIn(config: ViewerConfig): ViewerConfig {
  const currentIndex = ZOOM_LEVELS.indexOf(config.zoomLevel);
  if (currentIndex === ZOOM_LEVELS.length - 1) {
    return config;
  }
  return setZoomLevel(config, ZOOM_LEVELS[currentIndex + 1]);
}

export const imageViewerUtils = {
  setZoomLevel,
  zoomOut,
  resetZoom,
  zoomIn,
};
