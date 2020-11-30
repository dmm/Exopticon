/*
 * Exopticon - A free video surveillance system.
 * Copyright (C) 2020 David Matthew Mattli <dmm@mattli.us>
 *
 * This file is part of Exopticon.
 *
 * Exopticon is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Exopticon is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Exopticon.  If not, see <http://www.gnu.org/licenses/>.
 */

use crate::errors::ServiceError;
use crate::models::{
    AlertRule, AlertRuleCamera, AlertRuleModel, CreateAlertRule, DbExecutor, FetchAllAlertRule,
};
use actix::{Handler, Message};
use diesel::{self, prelude::*};

impl Message for CreateAlertRule {
    type Result = Result<AlertRule, ServiceError>;
}

impl Handler<CreateAlertRule> for DbExecutor {
    type Result = Result<AlertRule, ServiceError>;

    fn handle(&mut self, msg: CreateAlertRule, _: &mut Self::Context) -> Self::Result {
        use crate::schema::alert_rule_cameras::dsl::*;
        use crate::schema::alert_rules::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();

        conn.transaction::<_, ServiceError, _>(|| {
            let rule: AlertRule = diesel::insert_into(alert_rules)
                .values((
                    name.eq(msg.name),
                    analysis_instance_id.eq(msg.analysis_instance_id),
                    tag.eq(msg.tag),
                    details.eq(msg.details),
                    min_cluster_size.eq(msg.min_cluster_size),
                    cool_down_time.eq(msg.cool_down_time),
                    notifier_id.eq(msg.notifier_id),
                ))
                .get_result(conn)
                .map_err(|_error| ServiceError::InternalServerError)?;

            for c in msg.camera_ids {
                let child_camera = vec![(alert_rule_id.eq(rule.id), camera_id.eq(c))];
                diesel::insert_into(alert_rule_cameras)
                    .values(&child_camera)
                    .execute(conn)?;
            }
            Ok(rule)
        })
    }
}

impl Message for FetchAllAlertRule {
    type Result = Result<Vec<AlertRuleModel>, ServiceError>;
}

impl Handler<FetchAllAlertRule> for DbExecutor {
    type Result = Result<Vec<AlertRuleModel>, ServiceError>;

    fn handle(&mut self, _msg: FetchAllAlertRule, _: &mut Self::Context) -> Self::Result {
        use crate::schema::alert_rules::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();

        let rules = alert_rules
            .load::<AlertRule>(conn)
            .map_err(|_error| ServiceError::InternalServerError)?;

        let camera_ids = AlertRuleCamera::belonging_to(&rules)
            .load::<AlertRuleCamera>(conn)
            .map_err(|_error| ServiceError::InternalServerError)?
            .grouped_by(&rules);

        let models = rules
            .into_iter()
            .zip(camera_ids)
            .map(|(rule, camera_models)| {
                let camera_ids = camera_models.iter().map(|c| c.camera_id).collect();
                AlertRuleModel(rule, camera_ids)
            })
            .collect();

        Ok(models)
    }
}
