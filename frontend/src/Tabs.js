import React from 'react';
import connect from 'react-redux/es/connect/connect';
import Tab from 'semantic-ui-react/dist/commonjs/modules/Tab';
import ZonePicker from './ZonePicker';
import CollectibleList from './CollectibleList';
import { ActionCreators } from './actions';

const panes = [
    { menuItem: { key: 'Items', icon: 'list layout' }, render: () => <Tab.Pane><CollectibleList/></Tab.Pane> },
    { menuItem: { key: 'Zones', icon: 'map' }, render: () => <Tab.Pane><ZonePicker/></Tab.Pane> }
];

function mapStateToProps(state) {
    return { activeTab: state.selection.tab };
}

function Tabs(props) {

    const handleTabChange = (event, data) => {
        props.updateSelectedTab(data.activeIndex);
    };

    return (
        <Tab activeIndex={props.activeTab}
             onTabChange={handleTabChange}
             menu={{ borderless: true, attached: false, tabular: false, secondary: true }}
             panes={panes} />
    );
}

export default connect(mapStateToProps, ActionCreators)(Tabs);