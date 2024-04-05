import React, { useState } from "react";
import hover from "../../../assets/images/hover.png";
import { winner } from "../Utils/Data/SvgData";
import girl from "../../../assets/images/girl.jpeg";
import { useSelector } from "react-redux";
import toast, { Toaster } from "react-hot-toast";
const SecondEventCard = ({ data }) => {
  if (!data) {
    return null
  }

  let image = hover;
  let name = data?.cohort?.title ?? ""
  let launch_date = data?.cohort?.cohort_launch_date ?? ""
  let end_date = data?.cohort?.cohort_end_date ?? ""
  let deadline = data?.cohort?.deadline ?? ""
  let desc = data?.cohort?.description ?? ""
  let tags = data?.cohort?.tags ?? ""
  let seats = data?.cohort?.no_of_seats ?? 0

  const toastHandler = () => {
    toast.success("Thank you for the registration request. admin approval in process now.")
  }

  return (
    <>
      <div className="block w-full drop-shadow-xl rounded-lg bg-gray-200 mb-8">
        <div className="w-full relative">
          <img
            className="w-full object-cover rounded-lg "
            src={hover}
            alt="not found"
          />
          <div className="absolute h-12 w-12 -bottom-1 right-[20px]">
            {winner}
          </div>
        </div>
        <div className="w-full">
          <div className="p-8">
            <div className="w-full mt-4">
              <div className="w-1/2 flex-col text-[#737373] flex  ">
                <h1 className="font-bold text-black text-xl truncate">
                  {name}
                </h1>
                <p className="text-lg whitespace-nowrap">
                  {launch_date} - {end_date}
                </p>
              </div>
              <p className="text-[#7283EA] font-semibold text-xl">
                This event includes
              </p>
              <ul className="text-sm font-extralight list-disc list-outside pl-4">
                {tags && tags.split(",").map((val, index) => {
                  return (
                    <li>{val.trim()}</li>
                  )
                })}

              </ul>
              <div className="flex w-full py-2">
                <p className="line-clamp-3 min-h-4">{desc}</p>
              </div>
              <div className="flex flex-row flex-wrap space-x-8 mt-2">
                <div className="flex gap-4 justify-between w-full">
                  <div className="flex flex-col font-bold">
                    <p className="text-[#7283EA]">Deadline</p>
                    <p className="text-black whitespace-nowrap">{deadline}</p>
                  </div>
                  <div className="flex flex-col font-bold">
                    <p className="text-[#7283EA]">Seats</p>
                    <p className="flex text-black w-20">{seats}</p>
                  </div>
                  {/* <div className="flex flex-col font-bold">
                    <p className="text-[#7283EA]">Duration</p>
                    <p className="flex text-black w-20">60 min</p>
                  </div> */}
                </div>
              </div>
              <div className="flex justify-center items-center ">
                <button onClick={toastHandler} className="mb-2 uppercase w-full bg-[#3505B2] mr-2 text-white  px-4 py-2 rounded-md  items-center font-extrabold text-sm mt-2 ">
                  Register now
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <Toaster />
    </>
  );
};

export default SecondEventCard;
