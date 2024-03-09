import React, { useEffect, useState } from "react";
import { formFields } from "../../components/Utils/Data/userFormData";
import { useForm } from "react-hook-form";
import { yupResolver } from "@hookform/resolvers/yup";
import * as yup from "yup";
import { useDispatch, useSelector } from "react-redux";
// import { allHubHandlerRequest } from "../../StateManagement/Redux/Reducers/All_IcpHubReducer";
// import { AuthClient } from "@dfinity/auth-client";
import { ThreeDots } from "react-loader-spinner";
// import { useNavigate } from "react-router-dom";
import toast, { Toaster } from "react-hot-toast";
// import { userRoleHandler } from "../../StateManagement/Redux/Reducers/userRoleReducer";

const today = new Date();
const startDate = new Date("1900-01-01");

const schema = yup.object({
  full_name: yup
    .string()
    .required()
    .test("is-non-empty", null, (value) => value && value.trim().length > 0),
  user_name: yup
    .string()
    .min(6, "Username must be at least 6 characters")
    .max(20, "Username must be at most 20 characters")
    .matches(
      /^(?=.*[A-Z0-9_])[a-zA-Z0-9_]+$/,
      "Username can only contain letters, numbers, and underscores"
    )
    .required("Username is required"),
  bio: yup.string().required("is-non-empty"),
  email: yup.string().email().required(),
  telegram_id: yup.string().required().url(),
  twitter_id: yup.string().required().url(),
  hub: yup.string().required("Selecting a hub is required."),
  areas_of_expertise: yup
    .string()
    .required("Selecting a interest is required."),
});

const NormalUser = () => {
  const getAllIcpHubs = useSelector((currState) => currState.hubs.allHubs);
  const areaOfExpertise = useSelector(
    (currState) => currState.expertiseIn.expertise
  );
  const actor = useSelector((currState) => currState.actors.actor);
  const userFullData = useSelector((currState) => currState.userData);

  const [inputType, setInputType] = useState("date");

  const dispatch = useDispatch();
  //   const navigate = useNavigate();

  console.log("userInfo run =>", userFullData);
  //   console.log("getAllIcpHubs", getAllIcpHubs);

  //   useEffect(() => {
  //     dispatch(allHubHandlerRequest());
  //   }, [actor, dispatch]);

  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
    trigger,
    reset,
  } = useForm({
    resolver: yupResolver(schema),
    mode: "all",
  });

  const onSubmitHandler = async (data) => {
    // console.log("data aaya data aaya ", data);

    const userData = {
      full_name: [data.full_name],
      user_name: [data.user_name],
      bio: [data.bio],
      email: [data.email],
      telegram_id: [data.telegram_id.toString()],
      twitter_id: [data.twitter_id.toString()],
      country: [data.hub],
      area_of_intrest: [data.areas_of_expertise],
    };

    console.log("userData => ", userData);

    try {
      const result = await actor.register_user_role(userData);
      toast.success(result);
      console.log("data passed to backend");
      await dispatch(userRoleHandler());
      await navigate("/dashboard");
    } catch (error) {
      toast.error(error);
      console.error("Error sending data to the backend:", error);
    }
  };

  const handleFocus = (field) => {
    if (field.onFocus) {
      setInputType(field.onFocus);
    }
  };

  const handleBlur = (field) => {
    if (field.onBlur) {
      setInputType(field.onBlur);
    }
  };

  return (
    <div>
      <form onSubmit={handleSubmit(onSubmitHandler)} className="w-full px-4">
        <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
          {formFields?.map((field) => (
            <div key={field.id} className="relative z-0 group mb-6">
              <label
                htmlFor={field.id}
                className="block mb-2 text-lg font-medium text-gray-500 hover:text-black hover:whitespace-normal truncate overflow-hidden hover:text-left"
              >
                {field.label}
              </label>
              <input
                type={field.id === "date_of_birth" ? inputType : field.type}
                name={field.name}
                id={field.id}
                {...register(field.name)}
                className={`bg-gray-50 border-2 ${
                  errors[field.name]
                    ? "border-red-500 placeholder:text-red-500"
                    : "border-[#737373]"
                } text-gray-900 placeholder-gray-500 placeholder:font-bold text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5`}
                placeholder={field.placeholder}
                onFocus={() => handleFocus(field)}
                onBlur={() => handleBlur(field)}
              />
              {errors[field.name] && (
                <span className="mt-1 text-sm text-red-500 font-bold">
                  {errors[field.name].message}
                </span>
              )}
            </div>
          ))}

          <div className="relative z-0 group">
            <label
              htmlFor="hub"
              className="block mb-2 text-lg font-medium text-gray-700 hover:whitespace-normal truncate overflow-hidden hover:text-left"
            >
              Can you please share your preferred ICP Hub
            </label>
            <select
              {...register("hub")}
              id="hub"
              className={`bg-gray-50 border-2 ${
                errors.hub
                  ? "border-red-500 placeholder:text-red-500"
                  : "border-[#737373]"
              } text-gray-900 placeholder-gray-500 placeholder:font-bold text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5`}
            >
              <option className="text-lg font-bold" value="">
                Select your ICP Hub
              </option>
              {getAllIcpHubs?.map((hub) => (
                <option
                  key={hub.id}
                  value={`${hub.name} ,${hub.region}`}
                  className="text-lg font-bold"
                >
                  {hub.name} , {hub.region}
                </option>
              ))}
            </select>
            {errors.hub && (
              <span className="mt-1 text-sm text-red-500 font-bold">
                {errors.hub.message}
              </span>
            )}
          </div>
          <div className="z-0 w-full group">
            <label
              htmlFor="areas_of_expertise"
              className="block mb-2 text-lg font-medium text-gray-500 hover:text-black hover:whitespace-normal truncate overflow-hidden text-start"
            >
              What are your interests?
            </label>
            <select
              {...register("areas_of_expertise")}
              className={`bg-gray-50 border-2 ${
                errors.areas_of_expertise
                  ? "border-red-500 placeholder:text-red-500"
                  : "border-[#737373]"
              } text-gray-900 placeholder-gray-500 placeholder:font-bold text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5`}
            >
              <option className="text-lg font-bold" value="">
                interests ⌄
              </option>
              {areaOfExpertise?.map((expert) => (
                <option
                  key={expert.id}
                  value={`${expert.name}`}
                  className="text-lg font-bold"
                >
                  {expert.name}
                </option>
              ))}
            </select>
            {errors.areas_of_expertise && (
              <span className="mt-1 text-sm text-red-500 font-bold">
                {errors.areas_of_expertise.message}
              </span>
            )}
          </div>
        </div>
        <div className="flex justify-end mt-4">
          <button
            disabled={isSubmitting}
            type="submit"
            className="text-white font-bold bg-blue-800 hover:bg-blue-600 focus:ring-4 focus:outline-none focus:ring-blue-300 rounded-md w-auto sm:w-auto px-5 py-2 text-center mb-4"
          >
            {isSubmitting ? (
              <ThreeDots
                visible={true}
                height="35"
                width="35"
                color="#FFFEFF"
                radius="9"
                ariaLabel="three-dots-loading"
                wrapperStyle={{}}
                wrapperClass=""
              />
            ) : (
              "Submit"
            )}
          </button>
        </div>
      </form>
      <Toaster />
    </div>
  );
};

export default NormalUser;
