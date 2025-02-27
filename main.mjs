/**
 * @fileoverview Example of usage, for now
 *
 * @license MIT
 *
 * Licensed under the MIT License (see LICENSE file for details).
 *
 * SPDX-License-Identifier: MIT
 *
 * @author Pierre Sauvage <pierre@adaltas.com>
 */

import { openInDir, search, getDocumentByAddress } from "./index.js";

let index = openInDir(
  "/home/pierrotws/git/adaltas/alliage/knowledge-base/toto/",
);

let results = search(index, "incus");

for (const r of results) {
  console.log("----");
  console.log("score: " + r.score);
  console.log("doc_id: " + r.docAddress.docId);
  console.log("segment_ord: " + r.docAddress.segmentOrd);
  console.log("document: " + getDocumentByAddress(index, r.docAddress));
}
