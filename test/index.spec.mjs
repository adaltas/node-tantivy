import test from "ava";

import { openInDir, search, getDocumentByDocId } from "../index.js";

const TEST_DIR = "/home/pierrotws/git/adaltas/alliage/knowledge-base/toto/";

test("search function returns results within limit and with valid documents", (t) => {
  const index = openInDir(TEST_DIR);
  const query = "incus";
  const fields = ["content"];
  const limit = 20;

  const results = search(index, query, fields, limit);

  // Ensure results are not empty
  t.true(results.length > 0, "Results should not be empty");

  // Ensure results length does not exceed the limit
  t.true(
    results.length <= limit,
    `Results should not exceed the limit of ${limit}`,
  );

  // Ensure results are sorted by score in descending order
  for (let i = 1; i < results.length; i++) {
    t.true(
      results[i - 1].score >= results[i].score,
      "Results should be sorted in descending order by score",
    );
  }

  for (const r of results) {
    t.true(r.score > 0, "Each result must have a positive score");
    t.true(
      r.docAddress.docId > 0,
      "Each result must have a strict positive document ID",
    );
    t.not(r.docAddress.segmentOrd, undefined, "segmentOrd must be defined");

    const document = getDocumentByDocId(index, r.docAddress);
    t.truthy(document, "Each document must exist in the index");
  }
});
