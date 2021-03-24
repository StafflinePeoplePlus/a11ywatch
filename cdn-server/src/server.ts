/*
 * Copyright (c) A11yWatch, LLC. and its affiliates.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 **/

import { app, initApp } from "./app";
import {
  addScript,
  addScreenshot,
  getRoot,
  getFile,
  downloadScript,
  ROOT,
  ADD_SCRIPT,
  ADD_SCREENSHOT,
  DOWNLOAD_SCRIPT,
  GET_SCRIPT,
  GET_SCREENSHOT,
} from "./core/api";

app
  .get(ROOT, getRoot)
  .get(GET_SCRIPT, (req, res, next) => getFile(req, res, next, "scripts"))
  .get(GET_SCREENSHOT, getFile)
  .get(DOWNLOAD_SCRIPT, downloadScript)
  .post(ADD_SCRIPT, addScript)
  .post(ADD_SCREENSHOT, addScreenshot);

const server = initApp();

export const killServer = () => {
  return new Promise((resolve) => {
    server.close(() => {
      resolve(console.log("HTTP server closed"));
    });
  });
};

export { app as cdnServer };
