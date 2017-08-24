import React from 'react';
import connect from 'react-redux/es/connect/connect';
import Tab from 'semantic-ui-react/dist/commonjs/modules/Tab';
import ZonePicker from './ZonePicker';
import CollectibleList from './CollectibleList';
import { ActionCreators } from './actions';

const panes = [
    { menuItem: 'Items', render: () => <Tab.Pane><CollectibleList/></Tab.Pane> },
    { menuItem: 'Zones', render: () => <Tab.Pane><ZonePicker/></Tab.Pane> }
];

function mapStateToProps(state) {
    return { activeTab: state.view.tab };
}

function Tabs(props) {

    const handleTabChange = (event, data) => {
        props.updateTabView(data.activeIndex);
    };

    return (
        <Tab activeIndex={props.activeTab}
             onTabChange={handleTabChange}
             panes={panes} />
    );
}

export default connect(mapStateToProps, ActionCreators)(Tabs);