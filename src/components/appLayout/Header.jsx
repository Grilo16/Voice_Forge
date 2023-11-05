import styled from "styled-components";
import { VfLogo } from "../../assets";


export const Header = () => {
    return (
        <HeaderDiv>
            <VfLogo height={"4rem"} width={"10rem"}/>
        </HeaderDiv>
    )
};

const HeaderDiv = styled.div`
grid-area: header;
display: flex;
align-items: center;
justify-content: center;
background-color: #f3b96a;

background: linear-gradient(90deg, rgba(243,185,106,1) 0%, rgba(240,147,113,1) 100%);
`