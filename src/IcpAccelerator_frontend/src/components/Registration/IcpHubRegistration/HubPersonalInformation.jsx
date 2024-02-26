
import React from 'react';

const HubPersonalInformation = ({ onSubmit, register, errors, fields, goToNext }) => {
  return (
    <div>
      <form onSubmit={onSubmit} className="w-full px-4">
  <div className="grid grid-cols-1 sm:grid-cols-2 gap-6">
    {fields.map((field) => (
      <div key={field.id} className="relative z-0 group">
        <label htmlFor={field.id} className="block mb-2 text-sm font-medium text-gray-700 hover:whitespace-normal truncate overflow-hidden hover:text-left">{field.label}</label>
        <input
          type={field.type}
          name={field.name}
          id={field.id}
          {...register(field.name)}
          className={`bg-gray-50 border-2 ${errors[field.name] ? 'border-red-500 placeholder:text-red-500' : 'border-[#737373]'} text-gray-900 placeholder-gray-500 placeholder:font-bold text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5`}
          placeholder={field.placeholder}
        />
        {errors[field.name] && (
          <span className="mt-1 text-sm text-red-500 font-bold">{errors[field.name].message}</span>
        )}
      </div>
    ))}
  </div>

  <div className="flex justify-end">
    <button
      type="button"
      className="text-white font-bold bg-blue-500 hover:bg-blue-600 focus:ring-4 focus:outline-none focus:ring-blue-300 rounded-md w-auto sm:w-auto px-5 py-2 text-center mb-4"
      onClick={goToNext}
    >
      Next
    </button>
  </div>
</form>

    </div>
  );
};

export default HubPersonalInformation;