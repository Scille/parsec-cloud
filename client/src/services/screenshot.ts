// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

export async function takeScreenshot(): Promise<Blob | undefined> {
  let track: MediaStreamTrack | undefined;
  try {
    const stream = await navigator.mediaDevices.getDisplayMedia({ video: true });
    track = stream.getVideoTracks()[0];
    // ImageCapture only works on chromium-based browsers
    const capture = new ImageCapture(track);
    // Would love to use `capture.takePhoto()` but it seems bugged right now
    const bitmap = await capture.grabFrame();
    const canvas = document.createElement('canvas');
    canvas.width = bitmap.width;
    canvas.height = bitmap.height;
    const ctx = canvas.getContext('2d');
    ctx?.drawImage(bitmap, 0, 0);
    return new Promise((resolve, reject) => {
      canvas.toBlob(
        (blob) => {
          if (blob) {
            resolve(blob);
          } else {
            reject(new Error('Failed to convert to canvas to blob'));
          }
        },
        'image/png',
        0.9,
      );
    });
  } catch (e: any) {
    window.electronAPI.log('error', `Could not take a screenshot: ${e.toString()}`);
  } finally {
    if (track) {
      track.stop();
    }
  }
}
