import styled from "styled-components";

export const Header = styled.header`
  height: 70px;
  background: #333;
  color: #eee;
`;

export const Contents = styled.div`
  display: flex;
  flex-direction: row;
  justify-content: space-between;
  height: 100%;
`;

export const Logo = styled.div`
  display: flex;
  flex-direction: row;
  align-items: center;
  height: 100%;
  font-size: 24px;
  letter-spacing: 1px;
`;

export const Nav = styled.nav`
  display: flex;
  flex-direction: row;
  align-items: center;
  height: 100%;
  font-size: 16px;

  ul {
    padding: 0;
    margin: 0;
    list-style: none;
    display: flex;
    flex-direction: row;

    li:not(:last-child) {
      margin-right: 30px;
    }
  }
`;