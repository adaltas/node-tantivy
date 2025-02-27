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
