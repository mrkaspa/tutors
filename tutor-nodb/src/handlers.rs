use super::model::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let course_count_for_user = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == new_course.tutor_id)
        .collect::<Vec<Course>>()
        .len();

    let new_course = Course {
        tutor_id: new_course.tutor_id,
        course_id: Some(course_count_for_user + 1),
        course_name: new_course.course_name.clone(),
        posted_time: Some(Utc::now().naive_utc()),
    };
    app_state.courses.lock().unwrap().push(new_course);
    HttpResponse::Ok().json("course added")
}

pub async fn get_courses_for_tutor(
    app_state: web::Data<AppState>,
    params: web::Path<usize>,
) -> HttpResponse {
    let tutor_id = params.0;

    let filtered = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .filter(|course| course.tutor_id == tutor_id)
        .collect::<Vec<Course>>();

    if filtered.len() > 0 {
        HttpResponse::Ok().json(filtered)
    } else {
        HttpResponse::Ok().json("No course found".to_string())
    }
}

pub async fn get_courses_detail(
    app_state: web::Data<AppState>,
    params: web::Path<(usize, usize)>,
) -> HttpResponse {
    let (tutor_id, course_id) = params.0;

    let result = app_state
        .courses
        .lock()
        .unwrap()
        .clone()
        .into_iter()
        .find(|course| course.tutor_id == tutor_id && course.course_id == Some(course_id));

    if let Some(course) = result {
        HttpResponse::Ok().json(course)
    } else {
        HttpResponse::Ok().json("No course found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {
        let course = web::Json(Course {
            tutor_id: 1,
            course_name: "Hello, this is test course".into(),
            course_id: None,
            posted_time: None,
        });
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![]),
        });
        let resp = new_course(course, app_state).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success_test() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![Course {
                tutor_id: 1,
                course_name: "Hello, this is test course".into(),
                course_id: None,
                posted_time: None,
            }]),
        });
        let tutor_id: web::Path<usize> = web::Path::from(1);
        let resp = get_courses_for_tutor(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_success_test() {
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(vec![Course {
                tutor_id: 1,
                course_name: "Hello, this is test course".into(),
                course_id: Some(100),
                posted_time: None,
            }]),
        });
        let ids: web::Path<(usize, usize)> = web::Path::from((1, 100));
        let resp = get_courses_detail(app_state, ids).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
