import React from 'react';

const GradientButton = ({ CustomCss, children }) => {

    return (
        <button className=" h-[60px] w-[150px] button-gradient-wrapper text-white font-[700] text-base font-cabin rounded-lg py-4 px-[1.875rem] ">
            <span className="button-gradient-content p-4">
                {children}
            </span>
        </button>
    );
};

export default GradientButton;  