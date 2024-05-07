import { Dot } from 'lucide-react';
const DialogBox = ({ text }) => {
    return (
        <div className=" z-50 inset-0 flex justify-center items-center">
            <div className="bg-[#010427] text-white p-2 sm:p-4 rounded-lg shadow-xl font-cabin custom-text-size-14 sm:text-lg font-medium flex justify-center ">
                <span>
                    <Dot color='#F7931A' />
                </span>
                <span>{text}</span>
            </div>
        </div>
    );
};

export default DialogBox;
