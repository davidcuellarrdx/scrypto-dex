import React from "react";
import {
  ManifestBuilder,
  address,
  bucket,
  decimal,
} from "@radixdlt/radix-engine-toolkit";

const ManifestComponent = () => {
  const manifest = new ManifestBuilder()
    .callMethod(
      "account_sim1q3cztnp4h232hsfmu0j63f7f7mz5wxhd0n0hqax6smjqznhzrp",
      "withdraw",
      [
        address(
          "resource_sim1qf7mtmy9a6eczv9km4j4ul38cfvap0zy6juuj8m8xnxqlla6pd"
        ),
        decimal(10),
      ]
    )
    .takeAllFromWorktop(
      "resource_sim1qf7mtmy9a6eczv9km4j4ul38cfvap0zy6juuj8m8xnxqlla6pd",
      (builder, bucketId) =>
        builder.callMethod(
          "account_sim1qs5mg6tcehg95mugc9d3lpl90clnl787zmhc92cf04wqvqvztr",
          "try_deposit_or_abort",
          [bucket(bucketId)]
        )
    )
    .build();

  return (
    <div>
      <h2>Manifest</h2>
      <pre>{manifest.toString()}</pre>
    </div>
  );
};

export default ManifestComponent;
