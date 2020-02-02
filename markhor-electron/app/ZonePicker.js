import React from 'react';
import PropTypes from 'prop-types';
import { connect } from 'react-redux';
import ZoneItem from './ZoneItem';
import { ActionCreators } from './actions';
import List from 'semantic-ui-react/dist/commonjs/elements/List/List';

ZonePicker.propTypes = {
    zones: PropTypes.object.isRequired
};

function mapStateToProps(state) {
    return {
        activeZone: state.selection.zone,
        zones: state.zones
    };
}

function ZonePicker(props) {
    const listItems = props.zones
        .valueSeq()
        .sortBy(i => i.id)
        .map(item =>
            <List.Item key={item.id}>
                <ZoneItem activeZone={props.activeZone}
                          onActivate={props.updateSelectedZone}
                          item={item}/>
            </List.Item>);

    const noneItem = { name: 'None', id: null, description: null };

    return (
        <List>
            <List.Item>
                <ZoneItem activeZone={props.activeZone}
                          onActivate={props.updateSelectedZone}
                          item={noneItem}/>
            </List.Item>
            {listItems}
        </List>);
}

export default connect(mapStateToProps, ActionCreators)(ZonePicker);