import { PreviewTokenCard } from "./PreviewTokenCard";
import { TokenContainer } from "./TokenContainer";

export const TokenCreation = () => {
    return (
        <div className="grid grid-cols-1 lg:grid-cols-2 gap-10 w-full relative">
            <div className="h-full w-full border-r border-gray-200 relative justify-center hidden lg:flex">
                <PreviewTokenCard />
            </div>
            <TokenContainer />
        </div>
    );
}