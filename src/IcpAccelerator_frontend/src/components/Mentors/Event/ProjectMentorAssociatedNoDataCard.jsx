import React from "react";
import NoData from "../../../../assets/images/search_not_found.png";

function NoDataCard() {
  return (
    <div className="flex flex-col justify-center items-center w-full font-fontUse py-5">
      <div className="flex justify-center items-center">
        <img src={NoData} className="w-2/6" alt="" />
      </div>
      <p className="text-gray-400">You are not associated with any mentor yet</p>
    </div>
  );
}

export default NoDataCard;
