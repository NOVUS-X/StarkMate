import React, { ButtonHTMLAttributes, ReactNode } from 'react';

type ButtonVariant = 'primary' | 'secondary' | 'tertiary';
type ButtonSize = 'xs' | 'sm' | 'md' | 'lg';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
    children: ReactNode;
    variant?: ButtonVariant;
    size?: ButtonSize;
    leftIcon?: ReactNode;
    rightIcon?: ReactNode;
    isLoading?: boolean;
    bounceEffect?: boolean;
}
const Button: React.FC<ButtonProps> = ({
    children,
    variant = 'primary',
    bounceEffect = false,
    size = 'md',
    leftIcon,
    rightIcon,
    isLoading = false,
    className = '',
    disabled,
    onClick,
    ...rest
}) => {
    const isDisabled = disabled || isLoading
    const baseClasses = 'inline-flex items-center justify-center font-bold transition-all duration-300 focus:outline-none rounded-full text-white hover:cursor-pointer';
    const variantClasses = {
        primary: 'bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-700 hover:to-purple-700 hover:shadow-lg hover:shadow-indigo-500/30 ',
        secondary: 'bg-transparent border-2 border-purple-500 hover:bg-purple-500/20 transition-all duration-300 ',
        tertiary: "text-blue-400 hover:text-blue-300 transition-colors duration-200 font-medium"
    };
    const bounceEffectClass = bounceEffect ? 'transform hover:-translate-y-1' : '';

    const sizeClasses = {
        xs: 'py-1 px-2 text-xs',
        sm: 'py-2 px-4 text-sm',
        md: 'py-3 px-8 text-base',
        lg: 'py-4 px-10 text-lg'
    };

    const disabledClass = isDisabled ? 'opacity-50 pointer-events-none' : '';
    const buttonClasses = `${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${bounceEffectClass} ${disabledClass} ${className}`;

    return (
        <button
            className={buttonClasses}
            disabled={isDisabled}
            onClick={onClick}
            {...rest}
        >
            {isLoading && (
                <svg
                    className="animate-spin -ml-1 mr-2 h-4 w-4 text-current"
                    xmlns="http://www.w3.org/2000/svg"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <circle
                        className="opacity-25"
                        cx="12"
                        cy="12"
                        r="10"
                        stroke="currentColor"
                        strokeWidth="4"
                    ></circle>
                    <path
                        className="opacity-75"
                        fill="currentColor"
                        d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                    ></path>
                </svg>
            )}
            {leftIcon && <span className="mr-2">{leftIcon}</span>}
            {children}
            {rightIcon && <span className="ml-2">{rightIcon}</span>}
        </button>
    );
};

export default Button;