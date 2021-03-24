/*
 * Copyright (c) A11yWatch, LLC. and its affiliates.
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 **/

import { join } from "path";
import { log } from "@a11ywatch/log";
import { DEV, getFile as getAwsFile } from "../../";

const getFile = (req, res, next, pth?: string): void => {
  const url = `${pth || "screenshots"}/${req.params.domain}/${
    req.params.cdnPath
  }`;

  try {
    DEV
      ? res.sendFile(join(`${__dirname}/../../${url}`))
      : getAwsFile(url, res);
  } catch (e) {
    log(e);
    res.send(false);
  }
};

export { getFile };
